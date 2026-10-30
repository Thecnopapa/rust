import mmcif_parser
import os

filepath = "/home/iain/downloads/1M2Z.cif"
assert os.path.exists(filepath)
mmcif_parser.print(filepath)

mmcif = mmcif_parser.MMCIF(filepath)
print(mmcif)