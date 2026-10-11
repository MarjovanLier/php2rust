<?php

declare(strict_types=1);

function compare(float $a, float $b): void
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
    echo "\n";
}

compare(0.1 + 0.2, 0.3);
compare(1.5, 1.5);
compare(-0.0, 0.0);
compare(-2.5, 1e-3);
