<?php

declare(strict_types=1);

function show(string $text, string $characters): string
{
    return '[' . trim($text, $characters) . '|' . ltrim($text, $characters) . '|'
        . rtrim($text, $characters) . ']';
}

$texts = ['', 'x', 'xx', 'xaxbx', '..ab.cXYz..', "\t a b \n", "\0zz\0", 'abc', 'é-é', 'b..b'];
$lists = ['', 'x', 'xa', 'a..c', 'c..c.', '.', 'X..Z.', '...z', "\0..\x1f", ' a-z', 'zz..zz'];
foreach ($texts as $text) {
    foreach ($lists as $characters) {
        echo show($text, $characters), "\n";
    }
}
echo trim('--name--', '-'), ' ', ltrim('0012', '0'), ' ', rtrim("line\r\n", "\r\n"), "\n";
echo trim('abcHELLOcba', 'a..c'), ' ', rtrim('1.500', '0..9'), "\n";
