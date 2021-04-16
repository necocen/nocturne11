import React from "react";
import dayjs from "dayjs";
import { match, compile } from "path-to-regexp";

export function Calendar() {
    const path = window.location.pathname;
    const datePattern = String.raw`/:year(\d{4})-:month(\d{2}){-:day(\d{2})}?`;
    const dateMatcher = match<{ year: string; month: string; day?: string }>(datePattern);
    const dateToPath = compile<{ year: string; month: string; day?: string }>(datePattern);
    const dayjsToPath = (dayjs: dayjs.Dayjs, noDay: boolean = false) =>
        dateToPath({
            year: dayjs.format("YYYY"),
            month: dayjs.format("MM"),
            day: noDay ? undefined : dayjs.format("DD"),
        });
    const idMatcher = match<{ id: string }>(String.raw`/:id(\d+)`);
    const dateComponents = dateMatcher(path);
    const idComponents = idMatcher(path);

    let currentMonth = dayjs().startOf("month");
    if (dateComponents != false) {
        // 日付がある場合はそれを設定
        const { year, month } = dateComponents.params;
        currentMonth = dayjs(`${year}-${month}-01`);
    } else {
        // 日付はないがIDはある場合は記事のcreated-atを読み取る
        if (idComponents != false) {
            const id = idComponents.params.id;
            const post = document.getElementById(`post-${id}`);
            const createdAt = post?.querySelector("time.created-at")?.getAttribute("datetime");
            currentMonth = createdAt ? dayjs(createdAt).startOf("month") : currentMonth;
        }
    }

    const firstDayOnCalendar = currentMonth.startOf("week");
    const lastDayOnCalendar = currentMonth.endOf("month").endOf("week");
    const numberOfWeeks = lastDayOnCalendar.add(1, "day").diff(firstDayOnCalendar, "week");

    return (
        <table id="calendar" summary="calendar">
            <caption>&lt;&lt;{currentMonth.format("YYYY-MM")}&gt;&gt;</caption>
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
                                return <td key={day.format("YYYY-MM-DD")}>{day.format("D")}</td>;
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
