<?php

declare(strict_types=1);

/**
 * @param array{month: int, day: int, year: int} $parts
 */
function valid(array $parts): bool
{
    return checkdate($parts['month'], $parts['day'], $parts['year']);
}

$values = [-9223372036854775807 - 1, -400, -1, 0, 1, 2, 4, 12, 13, 28, 29, 30, 31, 32, 1900, 2000, 2023, 2024, 2400, 32767, 32768, 9223372036854775807];
$out = '';
foreach ($values as $month) {
    foreach ($values as $day) {
        foreach ($values as $year) {
            $out .= checkdate($month, $day, $year) ? '1' : '0';
        }
    }
    $out .= "\n";
}
echo $out;
echo valid(['month' => 2, 'day' => 29, 'year' => 2024]) ? 'yes' : 'no', "\n";
echo valid(['month' => 2, 'day' => 29, 'year' => 1900]) ? 'yes' : 'no', "\n";
