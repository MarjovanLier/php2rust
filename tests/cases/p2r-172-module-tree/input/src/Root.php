<?php

declare(strict_types=1);

namespace Acme\Lib;

use Acme\Lib\Dto\Dto10;
use Acme\Lib\Dto\Dto2;
use Acme\Lib\Enum\Kind;
use Acme\Lib\Type\Deep\Leaf;

final class Root
{
    public static function build(int $n): Dto2
    {
        return new Dto2(Leaf::twice($n));
    }

    public static function pick(Dto10 $dto): bool
    {
        return $dto->kind === Kind::Large;
    }
}
