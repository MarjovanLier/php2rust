<?php

declare(strict_types=1);

final class Stats
{
    public int $hits = 0;
    public float $mean = 0.5;
    public string $log = "";
    public ?int $best = null;
    /** @var list<int> */
    public array $seen = [];
    /** @var list<?int> */
    public array $maybe = [];
}

final class Report
{
    /** @var list<Stats> */
    public array $pages = [];

    public function __construct(public Stats $total)
    {
    }
}

$report = new Report(new Stats());
$report->total->hits = 10;
$report->total->hits += 5;
$report->total->hits -= 2;
$report->total->hits *= 3;
$report->total->hits++;
--$report->total->hits;
$report->total->mean++;
$report->total->mean = 2;
$report->total->log .= "a";
$hits = $report->total->hits;
$report->total->log .= "b{$hits}";
$report->total->best = 7;
$report->total->best = null;
$report->total->seen[] = 3;
$report->total->maybe[] = null;
$report->total->maybe[] = 4;
$report->pages[] = new Stats();
$report->pages[0]->hits = 1;
echo $report->total->hits, " ", $report->total->log, " ", count($report->total->seen), "\n";
echo count($report->total->maybe), " ", $report->pages[0]->hits, "\n";
