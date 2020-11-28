import "https://cdn.skypack.dev/intl?dts";
import { Temporal } from "https://cdn.skypack.dev/proposal-temporal?dts";

const july2023 = new Temporal.PlainYearMonth(2023, 7);

export type PlainDateFrom = Parameters<typeof Temporal.PlainDate.from>[0];

/**
 * returns whether I am bree on the given date
 * @param dateFrom a value that can be turned into a `Temporal.PlainDate` using `Temporal.PlainDate.from`
 */
export function is_bree(dateFrom: PlainDateFrom) {
  const date = Temporal.PlainDate.from(dateFrom);

  return (
    date.dayOfWeek === 5 &&
    Math.floor((date.day - 1) / 7) === 1 &&
    !(date.inLeapYear && date.month === 3) &&
    !date.toPlainYearMonth().equals(july2023)
  );
}
