<?php

declare(strict_types=1);

namespace Acme\Lib\Dto;

use Acme\Lib\Enum\Kind;

final readonly class Dto10
{
    public function __construct(public Kind $kind) {}
}
