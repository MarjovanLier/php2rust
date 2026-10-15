<?php

declare(strict_types=1);

$texts = [
    'hello world',
    'hello  world',
    "a\tb\nc\rd\fe\vf",
    'élan vital',
    'x-y_z',
    '123abc def',
    ' lead',
    'trail ',
    '',
    "\0a b",
    'ALREADY UP',
    'mIxEd cAsE',
];
foreach ($texts as $text) {
    echo '[', ucwords($text), "]\n";
}
