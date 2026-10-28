"""Expand Engage 2.0.0 inheritance prefabs without runtime object allocation."""
import argparse
import copy
import hashlib
import json
from pathlib import Path

import UnityPy
from UnityPy.streams import EndianBinaryReader

BASE = Path('Data/StreamingAssets/aa/Switch/fe_assets_ui/hub/godroom/prefabs')
TARGETS = {'unitselectroot.bundle': (-8783608894714709225, False),
           'godinfo.bundle': (-504617920136181681, False),
           'skillselectroot.bundle': (-6586472237546456982, True)}


def expand(raw, component_id, navigation, verify=True):
    env = UnityPy.load(raw)
    bundle = next(iter(env.files.values()))
    objects = {o.path_id: o for o in env.objects}
    original = {p: o.get_raw_data() for p, o in objects.items()}
    changed, added = set(), []
    sf = objects[component_id].assets_file
    next_id = max(objects) + 1

    def read(pid):
        return objects[pid].read_typetree(check_read=False)

    def save(pid, tree):
        obj = objects[pid]
        obj.save_typetree(tree)
        obj.reader = EndianBinaryReader(obj.data, endian=obj.reader.endian)
        obj.byte_start = 0
        obj.byte_size = len(obj.data)
        obj._read_until = None
        changed.add(pid)

    def ptr(pid):
        return {'m_FileID': 0, 'm_PathID': pid}

    def transform(go):
        return next(c['component']['m_PathID'] for c in read(go)['m_Component']
                    if objects[c['component']['m_PathID']].type.name in ('RectTransform', 'Transform'))

    def clone_tree(component):
        nonlocal next_id
        go = component if objects[component].type.name == 'GameObject' else read(component)['m_GameObject']['m_PathID']
        collected = []

        def collect(g):
            collected.append(g)
            collected.extend(c['component']['m_PathID'] for c in read(g)['m_Component'])
            for child in read(transform(g))['m_Children']:
                collect(read(child['m_PathID'])['m_GameObject']['m_PathID'])

        collect(go)
        mapping = {}
        for old in collected:
            mapping[old] = next_id
            next_id += 1

        def remap(value):
            if isinstance(value, dict):
                if value.get('m_FileID') == 0 and value.get('m_PathID') in mapping:
                    value['m_PathID'] = mapping[value['m_PathID']]
                else:
                    for item in value.values():
                        remap(item)
            elif isinstance(value, (list, tuple)):
                for item in value:
                    remap(item)

        for old, new in mapping.items():
            obj = copy.copy(objects[old])
            obj.path_id = new
            sf.objects[new] = obj
            objects[new] = obj
            tree = copy.deepcopy(read(old))
            remap(tree)
            save(new, tree)
            added.append(new)
        root_tf = transform(go)
        parent = read(root_tf)['m_Father']['m_PathID']
        parent_tree = read(parent)
        parent_tree['m_Children'].append(ptr(mapping[root_tf]))
        save(parent, parent_tree)
        return ptr(mapping[component])

    tree = read(component_id)
    sizes = {'m_ObjNaviList': 30} if navigation else {'m_Infos': 22, 'm_Wdws': 11}
    for field, count in sizes.items():
        entries = tree[field]
        assert len(entries) == (20 if navigation else 19 if field == 'm_Infos' else 10)
        template = entries[-1]['m_PathID']
        root = template if objects[template].type.name == 'GameObject' else read(template)['m_GameObject']['m_PathID']
        parent = read(transform(root))['m_Father']['m_PathID']
        while len(entries) < count:
            entries.append(clone_tree(template))
        parent_tree = read(parent)
        parent_tree['m_LocalScale']['y'] *= 2 / 3 if navigation else 915 / 935
        if not navigation:
            # Extend the panel downward by 65 UI units, keeping its top fixed.
            if field == 'm_Infos':
                parent_tree['m_SizeDelta']['y'] = 915.0
            panel_id = parent_tree['m_Father']['m_PathID']
            panel = read(panel_id)
            panel['m_SizeDelta']['y'] = 970.0
            save(panel_id, panel)
        if navigation:
            parent_tree['m_LocalScale']['x'] *= 2 / 3
        save(parent, parent_tree)
    save(component_id, tree)
    ab = next(o for o in objects.values() if o.type.name == 'AssetBundle')
    ab_tree = read(ab.path_id)
    ab_tree['m_PreloadTable'].extend(ptr(p) for p in added)
    for _, entry in ab_tree['m_Container']:
        assert entry['preloadIndex'] == 0
        entry['preloadSize'] = len(ab_tree['m_PreloadTable'])
    save(ab.path_id, ab_tree)
    result = bundle.save(packer='lz4')
    if not verify:
        return result, dict(array_sizes=sizes, panel_height=970 if not navigation else None,
                            validation='skipped-by-user-request',
                            sha256=hashlib.sha256(result).hexdigest())
    checked = {o.path_id: o for o in UnityPy.load(result).objects}
    assert set(checked) - set(original) == set(added)
    for pid, data in original.items():
        if pid not in changed:
            assert checked[pid].get_raw_data() == data, pid
    for pid in added:
        assert checked[pid].type_id == objects[pid].type_id
    def verify_refs(value):
        if isinstance(value, dict):
            if value.get('m_FileID') == 0 and value.get('m_PathID'):
                assert value['m_PathID'] in checked, value
            for item in value.values():
                verify_refs(item)
        elif isinstance(value, (list, tuple)):
            for item in value:
                verify_refs(item)
    for pid in changed:
        verify_refs(checked[pid].read_typetree(check_read=False))
    final = checked[component_id].read_typetree(check_read=False)
    assert all(len(final[f]) == n for f, n in sizes.items())
    return result, dict(array_sizes=sizes, added_objects=len(added),
                        changed_original_objects=sorted(set(original) & changed),
                        sha256=hashlib.sha256(result).hexdigest())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--romfs', type=Path, required=True)
    parser.add_argument('--output', type=Path, default=Path(__file__).resolve().parents[1] / 'resources/inheritance')
    parser.add_argument('--no-verify', action='store_true', help='Skip post-build validation')
    args = parser.parse_args()
    report = {}
    for name, (pid, nav) in TARGETS.items():
        source = args.romfs / BASE / name
        result, details = expand(source.read_bytes(), pid, nav, verify=not args.no_verify)
        target = args.output / BASE / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(result)
        report[name] = dict(source=str(source), source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(), **details)
        print(name, details, flush=True)
    (args.output / 'build-report.json').write_text(json.dumps(report, indent=2), encoding='utf-8')


if __name__ == '__main__':
    main()
