import React from "react";
import dayjs from "dayjs";
import useAxios from "axios-hooks";
import { useRouting } from "./routing";

export function Months() {
    // 記事のある月の一覧を取得
    const [{ data: { years } = { years: [] } }] = useAxios<{ years: { year: number; months: number[] }[] }>({
        url: "http://localhost:4000/api/months",
    });

    return (
        <>
            {years
                .sort((y1, y2) => y1.year - y2.year)
                .map((year) => (
                    <Year key={year.year} {...year} />
                ))}
        </>
    );
}

type YearProps = {
    year: number;
    months: number[];
};

function Year({ year, months }: YearProps) {
    const { dayjsToPath } = useRouting();
    const monthRows = [
        [1, 2, 3, 4, 5, 6],
        [7, 8, 9, 10, 11, 12],
    ].map((row) => row.map((m) => dayjs(`${year}-${m.toString().padStart(2, "0")}-01`)));
    return (
        <table className="year-month">
            <caption>{year}</caption>
            <tbody>
                {monthRows.map((monthRow, index) => (
                    <tr key={index}>
                        {monthRow.map((month) => (
                            <td key={month.format("YYYY-MM")}>
                                {months.includes(month.month() + 1) ? (
                                    <a href={dayjsToPath(month, true)}>{month.format("MM")}</a>
                                ) : (
                                    month.format("MM")
                                )}
                            </td>
                        ))}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
