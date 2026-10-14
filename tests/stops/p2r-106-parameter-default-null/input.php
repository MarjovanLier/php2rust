<?php

declare(strict_types=1);

function label(?string $s = "no" . "ne"): string
{
    if ($s === null) {
        return "none\n";
    }
    return "some\n";
}
