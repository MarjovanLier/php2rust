<?php

declare(strict_types=1);

function run(string $s, int $n, float $f): void
{
    echo sprintf('%5d', $n), "\n";
    echo sprintf('%1$s', $s), "\n";
    echo sprintf('%x', $n), "\n";
    echo sprintf('100%'), "\n";
    echo sprintf('%s %s', $s), "\n";
    echo sprintf('%d', $s), "\n";
    echo sprintf('%s', $f), "\n";
    echo sprintf($s, $n), "\n";
}
