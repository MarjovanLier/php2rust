<?php

declare(strict_types=1);

function whole(float $f): bool
{
    return in_array($f, [1, 2], true);
}
