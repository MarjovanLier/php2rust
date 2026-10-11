<?php

declare(strict_types=1);

function test(int $n, string $s, bool $flag): void
{
    if ($n) {
        echo 'n';
    }
    if ($flag && $n) {
        echo 'and';
    }
    if ($s || $flag) {
        echo 'or';
    }
    if (!$n) {
        echo 'not';
    }
}
