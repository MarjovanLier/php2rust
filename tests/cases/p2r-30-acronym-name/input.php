<?php

declare(strict_types=1);

function getHTTPCode(bool $ok): int
{
    if ($ok) {
        return 200;
    }
    return 500;
}

$isOK = getHTTPCode(true);
echo getHTTPCode(false), ' ', $isOK, "\n";
