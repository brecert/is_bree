import unittest

from datetime import date

from bree import is_bree

tests = (
	("not a friday", date(2020, 11, 25), False),
	("first friday of month", date(2020, 10, 2), False),
	("second friday of month", date(2020, 6, 12), True),
	("third friday of month", date(2020, 11, 20), False),
	("fourth friday of month", date(2020, 1, 24), False),
	("second friday, march, not leap year (1)", date(2019, 3, 8), True),
	("second frdiay, march, leap year (1)", date(2020, 3, 13), False),
	("second friday, march, not leap year (2)", date(1900, 3, 9), True),
	("second friday, march, leap year (2)", date(2000, 3, 10), False),
	("any other second friday, leap year", date(2000, 5, 12), True),
	("july 2023 friday", date(2023, 7, 7), False),
	("march 2023", date(2023, 3, 10), True),
	("october 2022", date(2022, 10, 14), True),
)

class IsBreeTest(unittest.TestCase):
	def test_is_bree(self):
		for name, day, expected in tests:
			with self.subTest(msg=name, day=day):
				self.assertEqual(is_bree(day), expected)