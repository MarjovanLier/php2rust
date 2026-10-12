<?php

declare(strict_types=1);

enum Word: string
{
    case Zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz = 'éééééééééééééééééééééééééé';
}

$w = Word::Zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz;
echo $w->value, "\n";
