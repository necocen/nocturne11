import React from "react";
import dayjs from "dayjs";
import {match, compile} from "path-to-regexp";

export function Calendar() {
    const path = window.location.pathname;
    const datePattern = String.raw`/:year(\d{4})-:month(\d{2}){-:day(\d{2})}?`;
    const dateMatcher = match<{year: string, month: string, day?: string}>(datePattern);
    const dateToPath = compile<{year: string, month: string, day?: string}>(datePattern);
    const dayjsToPath = (dayjs: dayjs.Dayjs, noDay: boolean = false) => dateToPath({
            year: dayjs.format("YYYY"),
            month: dayjs.format("MM"),
            day: noDay ? undefined : dayjs.format("DD"),
        });
    const idMatcher = match<{id: string}>(String.raw`/:id(\d+)`);
    const dateComponents = dateMatcher(path);
    const idComponents = idMatcher(path);

    let currentMonth = dayjs().startOf("month");
    if (dateComponents != false) {
        // 日付がある場合はそれを設定
        const {year, month} = dateComponents.params;
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

    console.log(dayjsToPath(currentMonth));

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
            <tr>
                <td className="calendar-past">28</td>
                <td className="calendar-past">29</td>
                <td className="calendar-past">30</td>
                <td className="calendar-past">31</td>
                <td><a href="./2021-04-01" title="diary on 2021-04-01">1</a></td>
                <td>2</td>
                <td><a href="./2021-04-03" title="diary on 2021-04-03">3</a></td>
            </tr>
            <tr>
                <td><a href="./2021-04-04" title="diary on 2021-04-04">4</a></td>
                <td><a href="./2021-04-05" title="diary on 2021-04-05">5</a></td>
                <td><a href="./2021-04-06" title="diary on 2021-04-06">6</a></td>
                <td>7</td>
                <td><a href="./2021-04-08" title="diary on 2021-04-08">8</a></td>
                <td><a href="./2021-04-09" title="diary on 2021-04-09">9</a></td>
                <td><a href="./2021-04-10" title="diary on 2021-04-10">10</a></td>
            </tr>
            <tr>
                <td><a href="./2021-04-11" title="diary on 2021-04-11">11</a></td>
                <td><a href="./2021-04-12" title="diary on 2021-04-12">12</a></td>
                <td><a href="./2021-04-13" title="diary on 2021-04-13">13</a></td>
                <td><a href="./2021-04-14" title="diary on 2021-04-14">14</a></td>
                <td>15</td>
                <td><a href="./2021-04-16" title="diary on 2021-04-16">16</a></td>
                <td>17</td>
            </tr>
            <tr>
                <td>18</td>
                <td>19</td>
                <td>20</td>
                <td>21</td>
                <td>22</td>
                <td>23</td>
                <td>24</td>
            </tr>
            <tr>
                <td>25</td>
                <td>26</td>
                <td>27</td>
                <td>28</td>
                <td>29</td>
                <td>30</td>
                <td className="calendar-future">1</td>
            </tr>
        </tbody>
    </table>
    );
}
