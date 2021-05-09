import React, { useState } from "react";
import useAxios from "axios-hooks";
import { useRouting } from "./routing";

export function Calendar() {
    const { thisMonth, dayjsToPath } = useRouting();
    const [currentMonth, setCurrentMonth] = useState(thisMonth);

    // 記事のある日付一覧を取得
    const [{ data: { days } = { days: undefined } }] = useAxios<{ days?: number[] }>({
        url: `http://localhost:4000/api/days/${currentMonth.format("YYYY-MM")}`,
    });

    const moveToPrevMonth = () => {
        setCurrentMonth(currentMonth.subtract(1, "month"));
    };
    const moveToNextMonth = () => {
        setCurrentMonth(currentMonth.add(1, "month"));
    };

    const firstDayOnCalendar = currentMonth.startOf("week");
    const lastDayOnCalendar = currentMonth.endOf("month").endOf("week");
    const numberOfWeeks = lastDayOnCalendar.add(1, "day").diff(firstDayOnCalendar, "week");

    return (
        <table id="calendar" summary="calendar">
            <caption>
                <button id="calendar-prev-month" onClick={moveToPrevMonth}><span>┗</span></button>
                <a href={dayjsToPath(currentMonth, true)}>{currentMonth.format("YYYY-MM")}</a>
                <button id="calendar-next-month" onClick={moveToNextMonth}><span>┓</span></button>
            </caption>
            <thead>
                <tr>
                    <th scope="col">Su</th>
                    <th scope="col">Mo</th>
                    <th scope="col">Tu</th>
                    <th scope="col">Wd</th>
                    <th scope="col">Th</th>
                    <th scope="col">Fr</th>
                    <th scope="col">Sa</th>
                </tr>
            </thead>
            <tbody>
                {[...Array(numberOfWeeks).keys()].map((w) => (
                    <tr key={`currentMonth.format("YYYY-MM")-W${w}`}>
                        {[...Array(7).keys()].map((d) => {
                            const day = firstDayOnCalendar.add(w * 7 + d, "day");
                            if (day.isSame(currentMonth, "month")) {
                                return (
                                    <td key={day.format("YYYY-MM-DD")}>
                                        {!days || days.includes(day.date()) ? <a href={dayjsToPath(day)}>{day.format("D")}</a> : day.format("D")}
                                    </td>
                                );
                            } else {
                                return <td key={day.format("YYYY-MM-DD")}></td>;
                            }
                        })}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
