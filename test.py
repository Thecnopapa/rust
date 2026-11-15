import mmcif_parser
import os, sys, json

filepath = "./test-cases/1M2Z.cif"
assert os.path.exists(filepath)
s = mmcif_parser.mmcif_to_json(filepath, [], ["_atom_site"], indent=4, verbose="--verbose" in sys.argv)
with open ("./test-output/rust.json", "w") as f:
    f.write(s)

#print(s)
j = json.loads(s)
# print(j)
# mmcif = mmcif_parser.MMCIF(filepath)
# print(mmcif)