use std::io::Read;

use crate::domain::ports::errors::AocClientError;
use crate::domain::ports::get_stars::GetStars;
use crate::domain::solved_parts::SolvedParts;
use crate::domain::stars::Stars;
use crate::infrastructure::aoc_api::AOC_URL;

use super::AocApi;

impl GetStars for AocApi {
    fn get_stars(&self, year: i32) -> Result<Stars, AocClientError> {
        let url = reqwest::Url::parse(&format!("{}/{}", AOC_URL, year))?;
        self.http_client
            .get(url)
            .send()?
            .error_for_status()?
            .try_into()
    }
}
impl Stars {
    fn parse_http_response(calendar_http_body: String) -> Result<Self, AocClientError> {
        let document = scraper::Html::parse_document(&calendar_http_body);
        let calendar_selector = scraper::Selector::parse(".calendar").unwrap();
        let calendar = match document.select(&calendar_selector).next() {
            Some(calendar) => calendar,
            None => return Err(AocClientError::GetStarsError),
        };
        let calendar_entries_selector =
            scraper::Selector::parse("[class^='calendar-day']:not(.calendar-day)").unwrap();
        let solved_parts = calendar
            .select(&calendar_entries_selector)
            .map(|day| {
                match (
                    day.value()
                        .classes()
                        .any(|class| class == "calendar-complete"),
                    day.value()
                        .classes()
                        .any(|class| class == "calendar-verycomplete"),
                ) {
                    (false, false) => SolvedParts::None,
                    (true, false) => SolvedParts::One,
                    (_, _) => SolvedParts::Both,
                }
            })
            .collect::<Vec<_>>();
        let calendar_entries = calendar
            .select(&calendar_entries_selector)
            .collect::<Vec<_>>();

        let entries_without_stars = std::iter::zip(solved_parts.clone(), calendar_entries)
            .map(|(solved_part, entry)| {
                let text = entry.text().collect::<Vec<_>>();
                Ok(match solved_part {
                    SolvedParts::Both => text.join(""),
                    SolvedParts::One => text
                        .join("")
                        .strip_suffix("*")
                        .ok_or_else(|| AocClientError::GetStarsError)?
                        .to_owned(),
                    SolvedParts::None => text
                        .join("")
                        .strip_suffix("**")
                        .ok_or_else(|| AocClientError::GetStarsError)?
                        .to_owned(),
                })
            })
            .collect::<Result<Vec<String>, AocClientError>>()?;

        Ok(Self::new(solved_parts, entries_without_stars))
    }
}

impl TryFrom<reqwest::blocking::Response> for Stars {
    type Error = AocClientError;

    fn try_from(mut value: reqwest::blocking::Response) -> Result<Self, Self::Error> {
        let mut body = String::from("");
        value.read_to_string(&mut body)?;

        Self::parse_http_response(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_no_stars_page() {
        let calendar = r#"
<pre class="calendar"><a aria-label="Day 25" href="/2019/day/25" class="calendar-day25">     <span class="calendar-s">.</span>             ''..     ':.              '.    <span class="calendar-day">25</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 24" href="/2019/day/24" class="calendar-day24">.......              <span class="calendar-s">.</span> ''.  <span class="calendar-s">.</span>  '.              :   <span class="calendar-day">24</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 23" href="/2019/day/23" class="calendar-day23">       '''''... <span class="calendar-s">.</span>         ''.    '.            <span class="calendar-s">.</span>'  <span class="calendar-day">23</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 22" href="/2019/day/22" class="calendar-day22">               ''..          '.  <span class="calendar-s">.</span> '.              <span class="calendar-day">22</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 21" href="/2019/day/21" class="calendar-day21">......   <span class="calendar-s">.</span><span class="calendar-s">.</span>        ''.         '.    '.            <span class="calendar-day">21</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 20" href="/2019/day/20" class="calendar-day20">      ''''...        <span class="calendar-s">.</span>'. <span class="calendar-s">.</span>       '.    :           <span class="calendar-day">20</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 19" href="/2019/day/19" class="calendar-day19">             ''..       '.         '.   '.  <span class="calendar-s">.</span><span class="calendar-s">.</span>     <span class="calendar-day">19</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 18" href="/2019/day/18" class="calendar-day18">.....       <span class="calendar-s">.</span>    ''.      '.<span class="calendar-s">.</span>       '.   '.        <span class="calendar-day">18</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 17" href="/2019/day/17" class="calendar-day17">  <span class="calendar-s">.</span>  ''''...  <span class="calendar-s">.</span>     '.      '.  <span class="calendar-s">.</span>    ': <span class="calendar-s">.</span> '.       <span class="calendar-day">17</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 16" href="/2019/day/16" class="calendar-day16">            '..  <span class="calendar-s">.</span>    '.      '.       :    :      <span class="calendar-day">16</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 15" href="/2019/day/15" class="calendar-day15">          <span class="calendar-s">.</span>    '.       '.     '.       :    :     <span class="calendar-day">15</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 14" href="/2019/day/14" class="calendar-day14">'''''...         '. <span class="calendar-s">.</span> <span class="calendar-s">.</span>  '.     '.       :   '.    <span class="calendar-day">14</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 13" href="/2019/day/13" class="calendar-day13">      <span class="calendar-s">.</span> ''..       '.     '.     '.      '.  <span class="calendar-s">.</span>:    <span class="calendar-day">13</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 12" href="/2019/day/12" class="calendar-day12">''''...  <span class="calendar-s">.</span><span class="calendar-s">.</span> '.      '.     '. <span class="calendar-s">.</span>   '.      :    :<span class="calendar-s">.</span>  <span class="calendar-day">12</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 11" href="/2019/day/11" class="calendar-day11">       ''.    '.   <span class="calendar-s">.</span>  :     '. <span class="calendar-s">.</span>   :      '. <span class="calendar-s">.</span> :   <span class="calendar-day">11</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 10" href="/2019/day/10" class="calendar-day10">'''''..   '.   '.      :     :  <span class="calendar-s">.</span>  '.      :   '.  <span class="calendar-day">10</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 9" href="/2019/day/9" class="calendar-day9">       '.  '.   '.     '.     :     :      :    :  <span class="calendar-day"> 9</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 8" href="/2019/day/8" class="calendar-day8">         :  '. <span class="calendar-s">.</span> :      :     :     :     <span class="calendar-s">.</span> :   :  <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 7" href="/2019/day/7" class="calendar-day7">'''.      : <span class="calendar-s">.</span>:   :      :     :     :   <span class="calendar-s">.</span>   :   :  <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 6" href="/2019/day/6" class="calendar-day6">   :   <span class="calendar-s">.</span>  :  :   :     <span class="calendar-s">.</span>:     : <span class="calendar-s">.</span>   :       :   :  <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 5" href="/2019/day/5" class="calendar-day5">...'      :  :   :      :     :     :       :   :  <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 4" href="/2019/day/4" class="calendar-day4">         :  .'   :      :     :<span class="calendar-s">.</span>    :  <span class="calendar-s">.</span>    :   :  <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 3" href="/2019/day/3" class="calendar-day3">     <span class="calendar-s">.</span> .'  .'   .'     .'<span class="calendar-s">.</span>    :   <span class="calendar-s">.</span> :      :    :  <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 2" href="/2019/day/2" class="calendar-day2">.....''   .'   .'      :     :     .'<span class="calendar-s">.</span>    <span class="calendar-s">.</span>:<span class="calendar-s">.</span>  .'  <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 1" href="/2019/day/1" class="calendar-day1">       ..'    .'  <span class="calendar-s">.</span>   :<span class="calendar-s">.</span>    .'     :     <span class="calendar-s">.</span>.'   :   <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
        </pre>"#.to_owned();

        Stars::parse_http_response(calendar);
    }

    #[test]
    fn parsing_partial_completion_one_star() {
        let calendar = r#"
<pre class="calendar"><a aria-label="Day 25" href="/2018/day/25" class="calendar-day25">                                                   <span class="calendar-day">25</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 24" href="/2018/day/24" class="calendar-day24">                                                   <span class="calendar-day">24</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 23" href="/2018/day/23" class="calendar-day23">                                                   <span class="calendar-day">23</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 22" href="/2018/day/22" class="calendar-day22">                                                   <span class="calendar-day">22</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 21" href="/2018/day/21" class="calendar-day21">                                                   <span class="calendar-day">21</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 20" href="/2018/day/20" class="calendar-day20">                                                   <span class="calendar-day">20</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 19" href="/2018/day/19" class="calendar-day19">                                                   <span class="calendar-day">19</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 18" href="/2018/day/18" class="calendar-day18">                                                   <span class="calendar-day">18</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 17" href="/2018/day/17" class="calendar-day17">                                                   <span class="calendar-day">17</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 16" href="/2018/day/16" class="calendar-day16">                                                   <span class="calendar-day">16</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 15" href="/2018/day/15" class="calendar-day15">                                                   <span class="calendar-day">15</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 14" href="/2018/day/14" class="calendar-day14">                                                   <span class="calendar-day">14</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 13" href="/2018/day/13" class="calendar-day13">                                                   <span class="calendar-day">13</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 12" href="/2018/day/12" class="calendar-day12">                                                   <span class="calendar-day">12</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 11" href="/2018/day/11" class="calendar-day11">                                                   <span class="calendar-day">11</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 10" href="/2018/day/10" class="calendar-day10">                                                   <span class="calendar-day">10</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 9" href="/2018/day/9" class="calendar-day9">                                                   <span class="calendar-day"> 9</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 8" href="/2018/day/8" class="calendar-day8">                                                   <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 7" href="/2018/day/7" class="calendar-day7">                                                   <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 6" href="/2018/day/6" class="calendar-day6">                                                   <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 5" href="/2018/day/5" class="calendar-day5">                                                   <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 4" href="/2018/day/4" class="calendar-day4">                                                   <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 3" href="/2018/day/3" class="calendar-day3">                                                   <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 2" href="/2018/day/2" class="calendar-day2">                                                   <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 1, one star" href="/2018/day/1" class="calendar-day1 calendar-complete">                  _  _ __ ___ __ _  _              <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
</pre>"#.to_owned();
        Stars::parse_http_response(calendar);
    }
}
