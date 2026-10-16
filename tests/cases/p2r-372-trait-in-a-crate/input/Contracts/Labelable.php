<?php

declare(strict_types=1);

namespace App\Contracts;

interface Labelable
{
    public function label(): string;
}
