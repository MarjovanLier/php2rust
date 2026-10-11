<?php

declare(strict_types=1);

function repeat(string $text, int $times): string
{
    $out = '';
    for ($i = 0; $i < $times; $i++) {
        $out .= $text;
    }
    return $out;
}

function frame(string $text): string
{
    $line = repeat('-', strlen($text) + 4);
    return "$line\n| $text |\n$line";
}

function banner(string $title): string
{
    return frame($title);
}

$title = 'php2rust';
$box = banner($title);
echo $box, "\n";
$csv = '';
$items = ['a', 'b', 'c'];
foreach ($items as $k => $item) {
    if ($k > 0) {
        $csv .= ',';
    }
    $csv .= $item;
}
echo $csv, ' ', strlen($csv), "\n", frame($csv), "\n";
echo $csv, ' ', $title, "\n";
