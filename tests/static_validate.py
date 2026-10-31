"""Offline package/manifest consistency: cannot replace cargo or game integration."""
import json
import re
import tomllib
from pathlib import Path

root = Path(__file__).resolve().parents[1]
compat = json.loads((root / 'compatibility.json').read_text('utf-8'))
mod = root / 'mixins' / 'aurora_visuals'
manifest = tomllib.loads((mod / 'horizon-mixin.toml').read_text('utf-8'))
cargo = tomllib.loads((root / 'Cargo.toml').read_text('utf-8'))
source = (mod / 'runtime.rs').read_text('utf-8')
assert compat['game'] == '26.3'
assert compat['hooks'] == 46
for key in ('horizon_revision', 'horizon_mixins_revision'):
    assert re.fullmatch(r'[0-9a-f]{40}', compat[key])
assert manifest['mod']['id'] == 'aurora_visuals'
version = manifest['mod']['version']
assert version == cargo['package']['version'] == '0.5.0'
assert f'pub const VERSION: &str = "{version}";' in source
assert manifest['compatibility']['horizon_revision'] == compat['horizon_revision']
expected = {'list', 'settings', 'click', 'hud', 'tick', 'key', 'command', 'visuals', 'commands'}
assert {v['id'] for v in manifest['injection']} == expected
assert len(manifest['injection']) == len(expected)
for inj in manifest['injection']:
    assert inj['kind'] == 'HEAD'
    assert inj['target'].startswith('horizon_client::game::native_hooks::')
    assert (mod / inj['helper']).is_file()
    assert inj['bindings']
assert len(manifest['module']) == 1
assert all((mod / module['source']).is_file() for module in manifest['module'])
assert len(list(mod.glob('*.rs'))) == 10
assert 'impl Motion' in source and 'struct FrameHistory' in source
assert 'struct LightTrail' in source and 'trail_lights()' in source
assert 'fn aurora_visuals_world' in (mod / 'visuals.rs').read_text()
assert 'native_player()' in (mod / 'visuals.rs').read_text()
assert 'native_on(m::ID)' in (mod / 'visuals.rs').read_text()
assert any(x['id'] == 'visuals' and 'native_visuals' in x['target'] for x in manifest['injection'])
assert '[package]' in (root / 'Cargo.toml').read_text()
print('PASS: manifest, v46 hook contract, module references, version and source checks')
