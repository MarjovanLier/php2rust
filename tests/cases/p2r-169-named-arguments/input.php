<?php

declare(strict_types=1);

final class Box
{
    public function __construct(public int $width = 1, public int $height = 1) {}

    public function grow(int $by = 1, bool $wide = false): void
    {
        if ($wide) {
            $this->width += $by;
            return;
        }
        $this->height += $by;
    }

    public static function square(int $side = 2): Box
    {
        return new Box(width: $side, height: $side);
    }

    public function area(): int
    {
        return $this->width * $this->height;
    }
}

function resize(Box $box, int $by = 1, bool $wide = false, string $note = ""): string
{
    $box->grow(by: $by, wide: $wide);
    return $note . $box->area();
}

/**
 * @param list<int> $xs
 */
function glued(array $xs, string $glue = ","): string
{
    $out = "";
    foreach ($xs as $x) {
        $out .= $x . $glue;
    }
    return $out;
}

$box = new Box(height: 3);
echo resize($box, wide: true), "\n";
echo resize(box: $box, note: "area "), "\n";
echo Box::square(side: 4)->area(), " ", Box::square()->area(), "\n";
$list = [1, 2];
echo glued(xs: $list), " ", glued($list, glue: ";"), "\n";
$box->grow(wide: true);
echo $box->area(), "\n";
