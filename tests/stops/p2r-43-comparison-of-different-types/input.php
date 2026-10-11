<?php

declare(strict_types=1);

function mixed_types(int $a, string $s, bool $flag): void
{
    if ($a === $s) {
        echo 'never';
    }
    if ($flag == $a) {
        echo 'loose';
    }
    if ($flag < true) {
        echo 'bool order';
    }
}
