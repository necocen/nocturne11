import React from "react";
import dayjs from "dayjs";

export function Months() {
    return (
        <>
            <Year year={2019} />
            <Year year={2020} />
            <Year year={2021} />
        </>
    );
}

type YearProps = {
    year: number;
};

function Year({ year }: YearProps) {
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
                            <td key={month.format("YYYY-MM")}>{month.format("MM")}</td>
                        ))}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
