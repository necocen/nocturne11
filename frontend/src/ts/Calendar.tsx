import { useState } from "react";
import useAxios from "axios-hooks";
import { useRouting } from "./routing";
import dayjs from "dayjs";

const API_HOST = import.meta.env.MODE === "production" ? "" : "http://localhost:4000";

export function Calendar() {
    const { thisMonth, dayjsToPath } = useRouting();
    const [currentMonth, setCurrentMonth] = useState(thisMonth);

    // 記事のある日付一覧を取得
    const [
        {
            data: { days } = { days: undefined },
        },
    ] = useAxios<{ days?: number[] }>({
        url: `${API_HOST}/api/days/${currentMonth.format("YYYY-MM")}`,
    });
    const [
        {
            data: { years } = { years: [] },
        },
    ] = useAxios<{ years: { year: number; months?: number[] }[] }>({
        url: `${API_HOST}/api/months`,
    });
    const months = years
        .flatMap(({ year, months }) => months?.map((month) => dayjs(`${year}-${month}-01`)) ?? [])
        .sort((a, b) => a.unix() - b.unix());
    const hasPrevMonth = months.length > 0 && months[0].isBefore(currentMonth);
    const hasNextMonth = months.length > 0 && months[months.length - 1].isAfter(currentMonth);

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
                <button id="calendar-prev-month" type="button" onClick={moveToPrevMonth} disabled={!hasPrevMonth}>
                    <span>┗</span>
                </button>
                <a href={days && days.length > 0 ? dayjsToPath(currentMonth, true) : undefined}>{currentMonth.format("YYYY-MM")}</a>
                <button id="calendar-next-month" type="button" onClick={moveToNextMonth} disabled={!hasNextMonth}>
                    <span>┓</span>
                </button>
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
                                        <a href={days?.includes(day.date()) ? dayjsToPath(day) : undefined}>{day.format("D")}</a>
                                    </td>
                                );
                            } else {
                                return <td key={day.format("YYYY-MM-DD")} />;
                            }
                        })}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
