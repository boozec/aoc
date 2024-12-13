import scala.io.Source

object Day1 {
  def part1(lefts: Array[Int], rights: Array[Int]): Int = {
    lefts
      .zip(rights)
      .map { case (l, r) =>
        math.abs(l - r)
      }
      .sum
  }

  def part2(lefts: Array[Int], rights: Array[Int]): Int = {
    val rightsCount = rights.groupBy(identity).map { case (k, v) =>
      (k, v.size)
    }
    lefts
      .map(x => x * rightsCount.getOrElse(x, 0))
      .sum
  }

  def main(args: Array[String]): Unit = {
    val lines =
      Source
        .fromFile(args(0))
        .mkString
        .split("\n")
        .map(_.split("   ").map(_.toInt))

    val (lefts_, rights_) = lines.map(arr => (arr(0), arr(1))).unzip
    val lefts = lefts_.sorted
    val rights = rights_.sorted

    assert(part1(lefts, rights) == 3569916)
    assert(part2(lefts, rights) == 26407426)
  }
}

