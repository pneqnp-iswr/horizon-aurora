"""Rebuild deterministic source-format .hnmod like official native.py package."""
import hashlib
import json
import tomllib
import zipfile
from pathlib import Path

root = Path(__file__).resolve().parents[1]
compat = json.loads((root / 'compatibility.json').read_text('utf-8'))
moddir = root / 'mixins' / 'aurora_visuals'
config = tomllib.loads((moddir / 'horizon-mixin.toml').read_text('utf-8'))
assert config['compatibility']['horizon_revision'] == compat['horizon_revision']
mod_id = config['mod']['id']
version = config['mod']['version']
manifest = {
    'schema': 1,
    'id': mod_id,
    'version': version,
    'game': compat['game'],
    'horizon': compat['horizon_revision'],
    'mixins': compat['horizon_mixins_revision'],
    'hooks': compat['hooks'],
}
files = {'horizon-native.json': (json.dumps(manifest, indent=2, sort_keys=True) + '\n').encode()}
for path in sorted(moddir.iterdir()):
    if path.suffix in ('.rs', '.toml'):
        files[f'mixins/{mod_id}/{path.name}'] = path.read_bytes()
out = root.parent / f'{mod_id}-{version}.hnmod'
with zipfile.ZipFile(out, 'w', compression=zipfile.ZIP_DEFLATED) as z:
    for name, data in sorted(files.items()):
        info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
        info.compress_type = zipfile.ZIP_DEFLATED
        info.external_attr = 0o100644 << 16
        z.writestr(info, data)
with zipfile.ZipFile(out) as z:
    assert sorted(z.namelist()) == sorted(files)
    assert all(z.read(name) == data for name, data in files.items())
print('PACKAGED', out, 'size', out.stat().st_size, 'sha256', hashlib.sha256(out.read_bytes()).hexdigest())
