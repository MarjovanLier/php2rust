<?php

declare(strict_types=1);

function reads(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false || $date->format('Y') === '1900') {
        return $date === false ? 'none' : 'old';
    }
    return $date->format('Y');
}

function writes(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if (!$date instanceof DateTime || $date->format('y') === '00') {
        return 'skip';
    }
    $date = DateTime::createFromFormat('!Y', '1980');
    return $text;
}

function later(?int $value): int
{
    if ($value % 2 !== 0 || $value === null) {
        return -1;
    }
    return $value;
}
