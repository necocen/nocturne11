import React, { useState } from "react";
import dayjs from "dayjs";
import useAxios from "axios-hooks";
import { useRouting } from "./routing";

export function Months() {
    // 記事のある月の一覧を取得
    // デフォルトは2010年から現在まで（レイアウト崩れを防ぐためのものなので記事はない）
    const defaultYears = [...Array(dayjs().year() - 2010 + 1).keys()].map((y) => ({ year: y + 2010, months: undefined }));
    const { thisMonth } = useRouting();
    const [{ data: { years } = { years: defaultYears } }] = useAxios<{ years: { year: number; months?: number[] }[] }>({
        url: "/api/months",
    });
    const [expandedYear, setExpandedYear] = useState(thisMonth.year());

    return (
        <>
            {years
                .sort((y1, y2) => y1.year - y2.year)
                .map((year) => (
                    <Year key={year.year} {...year} expand={setExpandedYear} expanded={expandedYear == year.year} />
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
                <button onClick={() => expand(year)}>{year}</button>
            </caption>
            <tbody className={expanded ? "expanded" : undefined}>
                {monthRows.map((monthRow, index) => (
                    <tr key={index}>
                        {monthRow.map((month) => (
                            <td key={month.format("YYYY-MM")}>
                                <a
                                    href={months && months.includes(month.month() + 1) ? dayjsToPath(month, true) : undefined}
                                    onFocus={() => expand(year)}>
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
