<?php

declare(strict_types=1);

// Outside a class PHP throws an Error: no class scope is active.
echo self::LIMIT, "\n";
echo static::make(), "\n";
