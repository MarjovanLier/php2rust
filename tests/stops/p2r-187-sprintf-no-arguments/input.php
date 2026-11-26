<?php

declare(strict_types=1);

// PHP throws an ArgumentCountError here: sprintf() needs a format.
function blank(): string
{
    return sprintf();
}
