//! Missing native Loading textures: look up the actual converted identity in enabled mods.
//! The native Setup still handles every preloaded texture. No roster or Loading-mode hooks.
use std::{collections::HashMap, sync::Mutex};
use unity::{Array, Class, Il2CppString, SystemType};
use unity::il2cpp::MethodInfo;
type P = *mut u8;
const PNG_DIR: &str = "patches/icon/loading";
const BUNDLE_DIR: &str = "Data/StreamingAssets/aa/Switch/fe_assets_ui/loading/icons";
const PREFIX: &str = "UI/Loading/Icons/Dot_Run_";

#[derive(Clone, Copy)]
struct Cached { texture: usize, _texture_root: usize, _bundle_root: usize }
// GCHandle retains managed wrappers; DontUnloadUnusedAsset also protects native assets.
// Native Clear destroys per-slot materials. Never use a destroyed slot material as a template.
static CACHE: Mutex<Option<HashMap<String, Cached>>> = Mutex::new(None);
static TEMPLATE: Mutex<Option<Cached>> = Mutex::new(None);
static OWNED_SLOTS: Mutex<Option<HashMap<usize, usize>>> = Mutex::new(None);

unsafe fn read<T: Copy>(p:P,offset:usize)->T { std::ptr::read_unaligned(p.add(offset) as *const T) }
fn base()->usize { unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize} }
fn method(ns:&str,class:&str,name:&str,count:usize)->Option<&'static MethodInfo> {
    let c=Class::try_lookup(ns,class).ok()?;
    let m=c.raw().get_method_from_name(name,count)?;
    (!m.method_ptr.is_null()).then_some(m)
}
unsafe fn alloc(class:&str)->Option<P> {
    let c=Class::try_lookup("UnityEngine",class).ok()?;
    extern "C" { fn il2cpp_object_new(class:*const unity::il2cpp::Il2CppClass)->P; }
    let p=il2cpp_object_new(c.raw());(!p.is_null()).then_some(p)
}
unsafe fn destroy(object:P) {
    if object.is_null(){return;}
    if let Some(m)=method("UnityEngine","Object","Destroy",1) {
        let f:unsafe extern "C" fn(P,*const MethodInfo)=std::mem::transmute(m.method_ptr);f(object,m);
    }
}
unsafe fn root(object:P)->Option<usize> {
    let m=method("System.Runtime.InteropServices","GCHandle","Alloc",1)?;
    let f:unsafe extern "C" fn(P,*const MethodInfo)->usize=std::mem::transmute(m.method_ptr);
    let handle=f(object,m);(handle!=0).then_some(handle)
}
// Unity native lifetime is separate from IL2CPP GC lifetime.
unsafe fn alive(object:P)->bool {
    if object.is_null(){return false;}
    let Some(m)=method("UnityEngine","Object","op_Implicit",1) else{return false;};
    let f:unsafe extern "C" fn(P,*const MethodInfo)->bool=std::mem::transmute(m.method_ptr);f(object,m)
}
unsafe fn retain_native(object:P)->Option<()> {
    let get=method("UnityEngine","Object","get_hideFlags",0)?;
    let set=method("UnityEngine","Object","set_hideFlags",1)?;
    let f:unsafe extern "C" fn(P,*const MethodInfo)->i32=std::mem::transmute(get.method_ptr);
    let flags=f(object,get)|32; // HideFlags.DontUnloadUnusedAsset
    let f:unsafe extern "C" fn(P,i32,*const MethodInfo)=std::mem::transmute(set.method_ptr);f(object,flags,set);
    Some(())
}
unsafe fn unroot(handle:usize) {
    if handle==0{return;}
    if let Some(m)=method("System.Runtime.InteropServices","GCHandle","Free",0) {
        let mut value=handle;
        let f:unsafe extern "C" fn(*mut usize,*const MethodInfo)=std::mem::transmute(m.method_ptr);f(&mut value,m);
    }
}
unsafe fn discard(c:Cached) {unroot(c._texture_root);unroot(c._bundle_root);}
fn valid_key(key:&str)->bool {
    !key.is_empty() && key.len()<=96 && key.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'_'||b==b'-')
}
fn valid_png(bytes:&[u8])->bool {
    bytes.len()>=33 && bytes.len()<=512*1024 && bytes[..8]==[137,80,78,71,13,10,26,10]
        && &bytes[12..16]==b"IHDR" && bytes[16..20]==288u32.to_be_bytes()
        && bytes[20..24]==48u32.to_be_bytes()
}
unsafe fn from_png(bytes:&[u8])->Option<Cached> {
    if !valid_png(bytes){return None;}
    let ctor=method("UnityEngine","Texture2D",".ctor",2)?;
    let load=method("UnityEngine","ImageConversion","LoadImage",2)?;
    let data=Array::<u8>::from_slice(bytes)?;
    let texture=alloc("Texture2D")?;
    let f:unsafe extern "C" fn(P,i32,i32,*const MethodInfo)=std::mem::transmute(ctor.method_ptr);
    f(texture,288,48,ctor);
    let f:unsafe extern "C" fn(P,Array<u8>,*const MethodInfo)->bool=std::mem::transmute(load.method_ptr);
    if !f(texture,data,load){destroy(texture);return None;}
    if retain_native(texture).is_none(){destroy(texture);return None;}
    let Some(texture_root)=root(texture) else {destroy(texture);return None;};
    Some(Cached{texture:texture as usize,_texture_root:texture_root,_bundle_root:0})
}
unsafe fn from_bundle(key:&str,bytes:&[u8])->Option<Cached> {
    if bytes.len()>4*1024*1024 || !bytes.starts_with(b"UnityFS\0"){return None;}
    let load=method("UnityEngine","AssetBundle","LoadFromMemory_Internal",2)?;
    let get=method("UnityEngine","AssetBundle","LoadAsset_Internal",2)?;
    let data=Array::<u8>::from_slice(bytes)?;
    let f:unsafe extern "C" fn(Array<u8>,u32,*const MethodInfo)->P=std::mem::transmute(load.method_ptr);
    let bundle=f(data,0,load);if bundle.is_null(){return None;}
    let c=Class::try_lookup("UnityEngine","Texture2D").ok()?;
    let ty=SystemType::from_il2cpp_type(c.raw().get_type())?;
    let name=Il2CppString::new(format!("Assets/Project/Addressables/UI/Loading/Icons/Dot_Run_{key}.png"));
    let f:unsafe extern "C" fn(P,Il2CppString,SystemType,*const MethodInfo)->P=std::mem::transmute(get.method_ptr);
    let texture=f(bundle,name,ty,get);
    if texture.is_null(){unload(bundle);return None;}
    let w=method("UnityEngine","Texture","get_width",0)?;
    let h=method("UnityEngine","Texture","get_height",0)?;
    let width:unsafe extern "C" fn(P,*const MethodInfo)->i32=std::mem::transmute(w.method_ptr);
    let height:unsafe extern "C" fn(P,*const MethodInfo)->i32=std::mem::transmute(h.method_ptr);
    if width(texture,w)!=288||height(texture,h)!=48 {unload(bundle);return None;}
    if retain_native(texture).is_none(){unload(bundle);return None;}
    let Some(texture_root)=root(texture) else {unload(bundle);return None;};
    // Release the bundle container without destroying the retained texture.
    unload_keep_assets(bundle);
    Some(Cached{texture:texture as usize,_texture_root:texture_root,_bundle_root:0})
}
unsafe fn unload(bundle:P) {
    if let Some(m)=method("UnityEngine","AssetBundle","Unload",1) {
        let f:unsafe extern "C" fn(P,bool,*const MethodInfo)=std::mem::transmute(m.method_ptr);f(bundle,true,m);
    }
}
unsafe fn unload_keep_assets(bundle:P) {
    if let Some(m)=method("UnityEngine","AssetBundle","Unload",1) {
        let f:unsafe extern "C" fn(P,bool,*const MethodInfo)=std::mem::transmute(m.method_ptr);f(bundle,false,m);
    }
}
unsafe fn texture(key:&str)->Option<P> {
    let mut lock=CACHE.lock().ok()?;
    let cache=lock.get_or_insert_with(HashMap::new);
    if let Some(cached)=cache.get(key).copied(){
        if alive(cached.texture as P){return Some(cached.texture as P);}
        cache.remove(key);discard(cached);
    }
    // Preserve identity spelling for PNGs. Unity Bundle filenames use lowercase.
    let mut loaded=mods::read(format!("{PNG_DIR}/Dot_Run_{key}.png")).ok().and_then(|b|from_png(&b));
    if loaded.is_none() {
        let path=format!("{BUNDLE_DIR}/dot_run_{}.bundle",key.to_ascii_lowercase());
        loaded=mods::read(path).ok().and_then(|b|from_bundle(key,&b));
    }
    if let Some(c)=loaded {
        for (property,value) in [("set_filterMode",0),("set_wrapMode",1),("set_anisoLevel",1)] {
            if let Some(m)=method("UnityEngine","Texture",property,1) {
                let f:unsafe extern "C" fn(P,i32,*const MethodInfo)=std::mem::transmute(m.method_ptr);f(c.texture as P,value,m);
            }
        }
    }
    let c=loaded?;let result=c.texture as P;cache.insert(key.to_owned(),c);Some(result)
}
unsafe fn template(image:P,ctor:&MethodInfo)->Option<P> {
    let mut lock=TEMPLATE.lock().ok()?;
    if let Some(c)=*lock {
        if alive(c.texture as P){return Some(c.texture as P);}
        discard(c);*lock=None;
    }
    let getter=method("UnityEngine.UI","Graphic","get_material",0)?;
    let f:unsafe extern "C" fn(P,*const MethodInfo)->P=std::mem::transmute(getter.method_ptr);
    let source=f(image,getter);if !alive(source){return None;}
    let material=alloc("Material")?;
    let f:unsafe extern "C" fn(P,P,*const MethodInfo)=std::mem::transmute(ctor.method_ptr);f(material,source,ctor);
    if retain_native(material).is_none(){destroy(material);return None;}
    let Some(handle)=root(material) else {destroy(material);return None;};
    *lock=Some(Cached{texture:material as usize,_texture_root:handle,_bundle_root:0});Some(material)
}
// Restore the native template and remove the RawImage override before original Setup/Clear.
// Original Clear owns destruction of slot+0x28; the immutable template never lives there.
unsafe fn detach(slot:P) {
    if slot.is_null(){return;}
    let Ok(mut slots)=OWNED_SLOTS.lock() else{return;};
    let Some(map)=slots.as_mut() else{return;};
    let Some(owned)=map.remove(&(slot as usize)) else{return;};
    if read::<usize>(slot,0x28)!=owned{return;}
    let image=read::<P>(slot,0x18);if image.is_null(){return;}
    let saved=TEMPLATE.lock().ok().and_then(|c|c.map(|v|v.texture as P));
    if let Some(set)=method("UnityEngine.UI","Graphic","set_material",1) {
        let f:unsafe extern "C" fn(P,P,*const MethodInfo)=std::mem::transmute(set.method_ptr);
        f(image,saved.filter(|&p|alive(p)).unwrap_or(std::ptr::null_mut()),set);
    }
    if let Some(set)=method("UnityEngine.UI","RawImage","set_texture",1) {
        let f:unsafe extern "C" fn(P,P,*const MethodInfo)=std::mem::transmute(set.method_ptr);f(image,std::ptr::null_mut(),set);
    }
}
unsafe fn show(slot:P,texture:P)->Option<()> {
    let image=read::<P>(slot,0x18);let animator=read::<P>(slot,0x20);
    if image.is_null()||animator.is_null(){return None;}
    // Resolve every call before changing visibility or the slot material.
    // Material has two one-argument constructors. Select the actual Material overload.
    let class=Class::try_lookup("UnityEngine","Material").ok()?;
    let ctor=class.declared_methods().iter().copied().find(|m|m.get_name().as_deref()==Some(".ctor")&&m.parameters_count==1
        && m.get_parameters()[0].get_name().as_deref()==Some("source"))?;
    let set_texture=method("UnityEngine","Material","set_mainTexture",1)?;
    let set_material=method("UnityEngine.UI","Graphic","set_material",1)?;
    let set_image_texture=method("UnityEngine.UI","RawImage","set_texture",1)?;
    let enabled=method("UnityEngine","Behaviour","set_enabled",1)?;
    let animator_class=Class::try_lookup("UnityEngine","Animator").ok()?;
    let play=animator_class.declared_methods().iter().copied().find(|m|m.get_name().as_deref()==Some("Play")&&m.parameters_count==3
        && m.get_parameters()[0].get_name().as_deref()==Some("stateName"))?;
    let source=template(image,ctor)?;
    let mut owned=OWNED_SLOTS.lock().ok()?;
    let material=alloc("Material")?;
    let f:unsafe extern "C" fn(P,P,*const MethodInfo)=std::mem::transmute(ctor.method_ptr);f(material,source,ctor);
    let f:unsafe extern "C" fn(P,P,*const MethodInfo)=std::mem::transmute(set_texture.method_ptr);f(material,texture,set_texture);
    let old=read::<P>(slot,0x28);destroy(old);
    std::ptr::write_unaligned(slot.add(0x28) as *mut P,material);
    let barrier:unsafe extern "C" fn(P,P)=std::mem::transmute(base()+0x491fb0);barrier(slot.add(0x28),material);
    let f:unsafe extern "C" fn(P,P,*const MethodInfo)=std::mem::transmute(set_material.method_ptr);f(image,material,set_material);
    let f:unsafe extern "C" fn(P,P,*const MethodInfo)=std::mem::transmute(set_image_texture.method_ptr);f(image,texture,set_image_texture);
    owned.get_or_insert_with(HashMap::new).insert(slot as usize,material as usize);
    let f:unsafe extern "C" fn(P,bool,*const MethodInfo)=std::mem::transmute(enabled.method_ptr);f(image,true,enabled);f(animator,true,enabled);
    let name=Il2CppString::new("IconAnim");let phase=((slot as usize>>4)%100) as f32/100.0;
    let f:unsafe extern "C" fn(P,Il2CppString,i32,f32,*const MethodInfo)=std::mem::transmute(play.method_ptr);
    f(animator,name,0,phase,play);Some(())
}
#[skyline::hook(offset=0x1fd7c60)]
unsafe fn fee_custom_loading_dot_setup_v2(slot:P,person:P,female:bool,mi:P)->bool {
    if slot.is_null()||person.is_null(){return false;}
    detach(slot);
    if call_original!(slot,person,female,mi){return true;}
    let get_path:unsafe extern "C" fn(P,bool,P)->P=std::mem::transmute(base()+0x1fd7eb0);
    let path=get_path(person,female,std::ptr::null_mut());if path.is_null(){return false;}
    let len=read::<i32>(path,0x10);if !(0..=160).contains(&len){return false;}
    let path=String::from_utf16_lossy(std::slice::from_raw_parts(path.add(0x14) as *const u16,len as usize));
    let Some(key)=path.strip_prefix(PREFIX).filter(|k|valid_key(k)) else{return false;};
    let Some(texture)=texture(key) else{return false;};
    show(slot,texture).is_some()
}
#[skyline::hook(offset=0x1fd7fd0)]
unsafe fn fee_custom_loading_dot_clear_v2(slot:P,mi:P) {
    detach(slot);call_original!(slot,mi);
}
pub fn install() {
    let guards=[(0x1fd7c60,0xa9bc7bfd),(0x1fd7c64,0xa9015ff8),(0x1fd7c68,0x910003fd),
        (0x1fd7eb0,0xa9bd7bfd),(0x1fd7eb4,0xf9000bf5),
        (0x1fd7fd0,0xa9bd7bfd),(0x1fd7fd4,0xa90157f6)];
    if guards.iter().all(|&(off,word)|unsafe{std::ptr::read_unaligned((base()+off) as *const u32)==word}) {
        skyline::install_hooks!(fee_custom_loading_dot_setup_v2,fee_custom_loading_dot_clear_v2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn identity_paths_are_bounded(){assert!(valid_key("555Lumiere"));assert!(valid_key("MOD_SHEZ"));assert!(!valid_key("../555Lumiere"));assert!(!valid_key(""));}
    #[test] fn reject_wrong_sheet_size(){let mut b=vec![0;33];b[..8].copy_from_slice(&[137,80,78,71,13,10,26,10]);b[12..16].copy_from_slice(b"IHDR");b[16..20].copy_from_slice(&288u32.to_be_bytes());b[20..24].copy_from_slice(&48u32.to_be_bytes());assert!(valid_png(&b));b[16..20].copy_from_slice(&48u32.to_be_bytes());assert!(!valid_png(&b));}
}
