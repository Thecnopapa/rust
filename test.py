import mmcif_parser
import os

filepath = "./test-cases/1M2Z.cif"
assert os.path.exists(filepath)
mmcif_parser.parse(filepath, ["all"], ["none"])

# mmcif = mmcif_parser.MMCIF(filepath)
# print(mmcif)