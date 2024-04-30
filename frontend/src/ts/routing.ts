import dayjs from "dayjs";
import { compile, match } from "path-to-regexp";

export function useRouting() {
    const path = window.location.pathname;
    const datePattern = String.raw`/:year(\d{4})-:month(\d{2}){-:day(\d{2})}?`;
    const dateMatcher = match<{ year: string; month: string; day?: string }>(datePattern);
    const dateToPath = compile<{ year: string; month: string; day?: string }>(datePattern);
    const dayjsToPath = (dayjs: dayjs.Dayjs, noDay = false) =>
        dateToPath({
            year: dayjs.format("YYYY"),
            month: dayjs.format("MM"),
            day: noDay ? undefined : dayjs.format("DD"),
        });
    const idMatcher = match<{ id: string }>(String.raw`/:id(\d+)`);
    const dateComponents = dateMatcher(path);
    const idComponents = idMatcher(path);

    let thisMonth = dayjs().startOf("month");
    if (dateComponents !== false) {
        // 日付がある場合はそれを設定
        const { year, month } = dateComponents.params;
        thisMonth = dayjs(`${year}-${month}-01`);
    } else {
        // 日付はないがIDはある場合は記事のcreated-atを読み取る
        if (idComponents !== false) {
            const id = idComponents.params.id;
            const post = document.getElementById(`post-${id}`);
            const createdAt = post?.querySelector("time.created-at")?.getAttribute("datetime");
            thisMonth = createdAt ? dayjs(createdAt).startOf("month") : thisMonth;
        }
    }

    return {
        thisMonth,
        dateMatcher,
        idMatcher,
        dayjsToPath,
    };
}
