# is_bree tool
# accepts a yyyy-mm-dd date as parameters and determines
# whether I am bree on that date

from datetime import date
from sys import argv as args, exit

from . import is_bree

def as_int(s):
	if not s.isdigit():
		print("'{}': not an integer".format(s))
		exit(-1)
	return int(s)

def main():
	if len(args) < 4:
		print("Usage: is_bree <yyyy> <mm> <dd>")
		exit(-1)
	try:
		day = date(*(as_int(args[i]) for i in range(1,4)))
	except ValueError:
		print("{}: not a date".format("-".join(args[1:4])))
		exit(-1)
	print("is_bree({!r}) = {}".format(day, is_bree(day)))