import mmcif_parser
import os

filepath = "./test-cases/1M2Z.cif"
assert os.path.exists(filepath)
print(mmcif_parser.mmcif_to_json(filepath, [], ["_atom_site"], 4))

# mmcif = mmcif_parser.MMCIF(filepath)
# print(mmcif)