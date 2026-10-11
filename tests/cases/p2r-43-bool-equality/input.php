<?php

declare(strict_types=1);

function compare(bool $a, bool $b): void
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
    echo "\n";
}

compare(true, true);
compare(true, false);
compare(false, false);
