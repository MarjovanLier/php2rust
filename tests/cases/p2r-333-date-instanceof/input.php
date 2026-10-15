<?php

declare(strict_types=1);

function year(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date instanceof DateTime) {
        return $date->format('Y');
    }
    return 'none';
}

function month(string $text): string
{
    $date = DateTime::createFromFormat('!m', $text);
    if (!$date instanceof DateTime) {
        return 'none';
    }
    return $date->format('n');
}

function day(string $text): string
{
    $date = DateTime::createFromFormat('!d', $text);
    if ($date !== false && $date->format('j') !== '1') {
        return $date->format('j');
    }
    return 'first or none';
}

function flags(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    $is = $date instanceof \DateTime;
    $not = !$date instanceof DateTime;
    return ($is ? 'is' : 'is not') . ' ' . ($not ? 'not' : 'date');
}

echo year('1980'), ' ', year('x'), "\n";
echo month('2'), ' ', month('13'), ' ', month(''), "\n";
echo day('15'), ' ', day('1'), ' ', day('x'), "\n";
echo flags('1980'), ', ', flags('x'), "\n";
