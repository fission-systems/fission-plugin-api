import idautils
import idc

for ea in idautils.Functions():
    print(hex(ea), idc.get_func_name(ea))
