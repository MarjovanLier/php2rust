<?php

declare(strict_types=1);

$table = [['id', 'name']];
$table[] = ['1', 'Ann'];
$table[] = ['2', 'Bob'];
foreach ($table as $r => $row) {
    $line = '';
    foreach ($row as $c => $cell) {
        if ($c > 0) {
            $line .= ' | ';
        }
        $line .= $cell;
    }
    echo $r, ': ', $line, "\n";
}
$last = count($table) - 1;
echo count($table), ' ', $table[$last][1], "\n";
