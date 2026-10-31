"""Checks Horizon shader pack structure, option coverage and file constraints.
Does not replace naga/translation validation (`horizon-shader check`)."""
from pathlib import Path
import tomllib
p=Path(__file__).resolve().parents[1]/'shaderpacks/aurora_nebula'
c=tomllib.loads((p/'horizon-shaders.toml').read_text('utf-8'))
assert c['schema']==1 and c['id']=='aurora_nebula' and c['api']=='0.1'
assert 1 <= len(c['post']) <= 8 and len(c['options']) <=64
stages=[c['sky'],*c['post']]
files=set()
for stage in stages:
    file=stage['file']
    assert '..' not in Path(file).parts and file.endswith('.wgsl')
    script=(p/file).read_text('utf-8')
    assert script.count('fn sky(' if stage is c['sky'] else 'fn post(')==1
    assert not any(forbidden in script for forbidden in ('@group','@binding','@vertex','@fragment','@compute'))
    assert len(script.encode())<256*1024
    files.add(file)
opts={o['id'] for o in c['options']}
assert len(opts)==len(c['options'])
for option in c['options']:
    assert option['type'] in ('bool','float','int','choice')
    occurrences=sum(f'override {option["id"]}:' in (p/file).read_text('utf-8') for file in files)
    assert occurrences==1,(option['id'],occurrences)
assert len(files)==len(stages)
assert len(list(p.rglob('*.wgsl')))==len(files)
print(f'PASS: shaderpack schema, {len(stages)} WGSL stages, {len(opts)} overrides and file limits (no naga validation)')
