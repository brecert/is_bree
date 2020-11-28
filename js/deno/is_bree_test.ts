import { assertEquals } from "https://deno.land/std@0.79.0/testing/asserts.ts";
import { is_bree, PlainDateFrom } from "./is_bree.ts";

const testDate = (name: string, date: PlainDateFrom, expected: boolean) =>
  Deno.test(name, () => assertEquals(is_bree(date), expected));

testDate("not a friday", "2020-11-25", false);
testDate("first friday of month", "2020-10-02", false);
testDate("second friday of month", "2020-06-12", true);
testDate("third friday of month", "2020-11-20", false);
testDate("fourth friday of month", "2020-01-24", false);
testDate("second friday, march, not leap year (1)", "2019-03-08", true);
testDate("second frdiay, march, leap year (1)", "2020-03-13", false);
testDate("second friday, march, not leap year (2)", "1900-03-09", true);
testDate("second friday, march, leap year (2)", "2000-03-10", false);
testDate("any other second friday, leap year", "2000-05-12", true);
testDate("july 2023", "2023-07-14", false);
testDate("march 2023", "2023-03-10", true);
