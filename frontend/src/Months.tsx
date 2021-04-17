import React from "react";
import dayjs from "dayjs";

export function Months() {
    return (
        <>
            <Month year={2019} />
            <Month year={2020} />
            <Month year={2021} />
        </>
    );
}

type MonthProps = {
    year: number;
};

function Month({ year }: MonthProps) {
    const monthRows = [
        [1, 2, 3, 4, 5, 6],
        [7, 8, 9, 10, 11, 12],
    ].map((row) => row.map((m) => dayjs(`${year}-${m.toString().padStart(2, "0")}-01`)));
    return (
        <table>
            <caption>{year}</caption>
            <tbody>
                {monthRows.map((monthRow, index) => (
                    <tr key={index}>
                        {monthRow.map((month) => (
                            <td key={month.format("YYYY-MM")}>{month.format("MM")}</td>
                        ))}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
