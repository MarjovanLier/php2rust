<?php

declare(strict_types=1);

function addTax(int $amount): int
{
    return $amount + intDiv($amount, 10);
}

/** @param list<string> $words */
function longest(array $words): int
{
    $best = 0;
    foreach ($words as $word) {
        if (StrLen($word) > $best) {
            $best = STRLEN($word);
        }
    }
    return $best;
}

$prices = [100, 250];
echo ADDTAX(50), ' ', addtax($prices[1]), ' ', AddTax(Count($prices)), "\n";
$words = ['a', 'abc', 'ab'];
echo LONGEST($words), ' ', INTDIV(9, 2), "\n";
