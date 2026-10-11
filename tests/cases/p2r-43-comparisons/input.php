<?php

declare(strict_types=1);

function compare(int $a, int $b, string $s, bool $flag): void
{
    if ($a === $b) {
        echo '===';
    }
    if ($a !== $b) {
        echo '!==';
    }
    if ($a == $b) {
        echo '==';
    }
    if ($a != $b) {
        echo '!=';
    }
    if ($a <> $b) {
        echo '<>';
    }
    if ($a < $b) {
        echo '<';
    }
    if ($a > $b) {
        echo '>';
    }
    if ($a <= $b) {
        echo '<=';
    }
    if ($a >= $b) {
        echo '>=';
    }
    if ($s === 'yes') {
        echo ' s===yes';
    }
    if ('no' !== $s) {
        echo ' s!==no';
    }
    if ($flag === true) {
        echo ' flag';
    }
    if ($flag == $a > $b) {
        echo ' flag==a>b';
    }
    if (($a < $b) === ($b < $a)) {
        echo ' same';
    }
    if ($a * 2 + 1 > $b - 3) {
        echo ' sum';
    }
    if (3000000000 < $a + 9000000000) {
        echo ' big';
    }
    echo "\n";
}

compare(1, 2, 'yes', true);
compare(2, 2, 'no', false);
compare(3, 2, 'maybe', true);
