import { useState } from "react";
import dayjs from "dayjs";
import useAxios from "axios-hooks";
import { useRouting } from "./routing";

const API_HOST = import.meta.env.MODE === "production" ? "" : "http://localhost:4000";

export function Months() {
    // 記事のある月の一覧を取得
    // デフォルトは2010年から現在まで（レイアウト崩れを防ぐためのものなので記事はない）
    const defaultYears = [...Array(dayjs().year() - 2010 + 1).keys()].map((y) => ({
        year: y + 2010,
        months: undefined,
    }));
    const { thisMonth } = useRouting();
    const [
        {
            data: { yearMonths } = { yearMonths: undefined },
        },
    ] = useAxios<{ yearMonths: { year: number; month: number }[] }>({
        url: `${API_HOST}/api/year_months`,
    });
    const [expandedYear, setExpandedYear] = useState(thisMonth.year());

    const years = yearMonths
        ? yearMonths
              .reduce((acc, { year, month }) => {
                  const yearData = acc.find((y) => y.year === year);
                  if (yearData) {
                      yearData.months.push(month);
                  } else {
                      acc.push({ year, months: [month] });
                  }
                  return acc;
              }, [] as { year: number; months: number[] }[])
              .map((yearData) => ({
                  ...yearData,
                  months: yearData.months.sort((m1, m2) => m1 - m2),
              }))
              .sort((y1, y2) => y1.year - y2.year)
        : defaultYears;

    return (
        <>
            {years.map((year) => (
                <Year key={year.year} {...year} expand={setExpandedYear} expanded={expandedYear === year.year} />
            ))}
        </>
    );
}

type YearProps = {
    year: number;
    months?: number[];
    expanded: boolean;
    expand: (year: number) => void;
};

function Year({ year, months, expand, expanded }: YearProps) {
    const { dayjsToPath } = useRouting();
    const monthRows = [
        [1, 2, 3, 4, 5, 6],
        [7, 8, 9, 10, 11, 12],
    ].map((row) => row.map((m) => dayjs(`${year}-${m.toString().padStart(2, "0")}-01`)));
    return (
        <table className="year-month">
            <caption>
                <button type="button" onClick={() => expand(year)}>
                    {year}
                </button>
            </caption>
            <tbody className={expanded ? "expanded" : undefined}>
                {monthRows.map((monthRow) => (
                    <tr key={monthRow[0].month()}>
                        {monthRow.map((month) => (
                            <td key={month.format("YYYY-MM")}>
                                <a href={months?.includes(month.month() + 1) ? dayjsToPath(month, true) : undefined} onFocus={() => expand(year)}>
                                    {month.format("MM")}
                                </a>
                            </td>
                        ))}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
