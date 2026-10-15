<?php

declare(strict_types=1);

function write(string $text, string $format): string
{
    $date = DateTime::createFromFormat('!Y-m-d H:i:s', $text);
    if ($date === false) {
        return 'false';
    }
    return $date->format($format);
}

function fixed(string $text): string
{
    $date = DateTime::createFromFormat('!Y-m-d H:i:s', $text);
    if (!$date instanceof DateTime) {
        return 'false';
    }
    return $date->format('d/m/Y j.n.y G:i:s H \Y\\\\ *+!|#?');
}

$all = 'Y y m n d j H G i s';
echo write('1980-02-03 04:05:06', $all), "\n";
echo write('0000-01-00 00:00:00', $all), "\n";
echo write('0000-00-00 00:00:00', 'Y-m-d y'), "\n";
echo write('9999-12-31 23:59:61', $all), "\n";
echo write('0098-01-01 00:00:00', 'Y y'), ' ', write('0999-01-01 00:00:00', 'Y y'), "\n";
echo write('2000-10-05 13:07:09', '\D\a\y d'), ' ', write('2000-10-05 13:07:09', "Y\u{00E9}m"), "\n";
echo fixed('2023-07-04 09:08:07'), "\n";
echo fixed('1980-02-29 23:59:59'), "\n";
