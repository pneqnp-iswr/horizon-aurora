"""Deterministic ZIP pack candidate. This does NOT invoke official horizon-shader check."""
import zipfile,hashlib,tomllib
from pathlib import Path
p=Path(__file__).resolve().parents[1]/'shaderpacks/aurora_nebula'
m=tomllib.loads((p/'horizon-shaders.toml').read_text('utf-8'))
paths=sorted([f for f in p.rglob('*') if f.is_file()])
assert all(f.suffix in ('.wgsl','.toml','.md') or f.name=='LICENSE' for f in paths)
out=p.parents[2]/f'{m["id"]}-{m["version"]}.hshader'
with zipfile.ZipFile(out,'w',compression=zipfile.ZIP_DEFLATED) as z:
    for f in paths:
        info=zipfile.ZipInfo(f.relative_to(p).as_posix(),(2026,1,1,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED;info.external_attr=0o100644<<16
        z.writestr(info,f.read_bytes())
with zipfile.ZipFile(out) as z:
    assert sorted(z.namelist())==sorted(f.relative_to(p).as_posix() for f in paths)
print('PACKAGED',out,out.stat().st_size,'sha256',hashlib.sha256(out.read_bytes()).hexdigest())
