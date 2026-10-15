<?php

declare(strict_types=1);

function check(string $format, string $text): string
{
    $date = DateTime::createFromFormat($format, $text);
    if ($date === false) {
        return 'false';
    }
    return $date->format('Y-m-d H:i:s');
}

echo check('!Ymd', '19800229'), "\n";
echo check('!Ymd', '19800230'), "\n";
echo check('!Ymd', '1980021'), "\n";
echo check('!Ymd', '0000101'), "\n";
echo check('!Ymd', '00000000'), "\n";
echo check('!Ymd', '198002011'), "\n";
echo check('!Ymd', '1980-1-1'), "\n";
echo check('!Ymd', ''), "\n";
echo check('!Y-m-d H:i:s', '9999-99-99 99:99:99'), "\n";
echo check('!Y-m-d H:i:s', '2023-12-25 25:00:00'), "\n";
echo check('!Y-m-d H:i:s', '2023-12-25 3:5:7'), "\n";
echo check('!j-n-Y G', '1-2-1980 7'), "\n";
echo check('!y-m-d', '69-01-01'), ' ', check('!y-m-d', '70-01-01'), ' ', check('!y', '5'), "\n";
echo check('!Y m d', "1980  01\t01"), ' ', check('!Y m d', '19800101'), "\n";
echo check('!Y m d', "1980\u{00A0}01\u{202F}01"), ' ', check('!Y m d', "1980\n01 01"), "\n";
echo check('!Y#m#d', '1980;01(01'), ' ', check('!Y#m', '1980#01'), "\n";
echo check('!Y?m', '1980x01'), ' ', check('!Y\Tm', '1980T01'), ' ', check('!Y\Tm', '1980x01'), "\n";
echo check('!Y_m', '1980_01'), ' ', check('!Y_m', '1980-01'), ' ', check('!Y\\', '1980'), "\n";
echo check('Y!m', '19802'), ' ', check('m|', '2'), ' ', check('Y-m-d|', '1980-02-03'), "\n";
echo check('!Ymd ', '19800101'), ' ', check('Ymd!', '19800101'), "\n";
echo check("!Y\0x", '1980'), ' ', check('!i', '5'), ' ', check('!H', '7'), "\n";
echo check('!d.m.Y', '01.02.1980'), ' ', check('!dmY', '311219'), "\n";
