<?php

declare(strict_types=1);

namespace App\Data;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Optional;

// A partial update of a task: a field the request leaves out stays Optional.
final class TaskChange extends Data
{
    public function __construct(
        public string|Optional $title,
        public bool|Optional $done,
    ) {}
}
