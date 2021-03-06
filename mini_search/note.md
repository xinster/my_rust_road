# 自制搜索引擎

倒排索引的制作和压缩、检索的处理流程以及搜索引擎的优化等内容

学习到如何高效地求得多个大集合的交集，如何压缩存储量大的整数，如何运用sar命令查看并分析系统的性能。


# 概念

搜索引擎技术应用于信息检索、数据库等领域的技术。


## 搜索引擎的构成

### 什么是搜索引擎

搜索引擎是一类系统或软件的统称。

搜索引擎的主要作用是 从文档的集合中查找（检索）出匹配信息需求（查询）的文档，信息需求是由单词、问题等构成。

本文所指的搜索引擎是指全文搜索引擎（Full text Search Engine）。所谓的“全文”指的就是全部的句子，当检索的对象为“由文本构成的文档中的全部句子”时，对于该文档进行得检索就称为**全文搜索**。实现了这种全文搜索的系统就是全文搜索引擎（全文搜索系统）。



### 构成搜索引擎的组件

- 索引管理器 Index Manger

- 索引检索器 Index Searcher

- 索引构建器 Indexer

- 文档管理器 Document Manger

* 索引管理器

索引管理器的作用，管理带有索引结构的数据，索引结构是一种用于进行高速检索的数据结构。
对于索引的访问也是通过索引管理器进行的。

索引管理通常是将索引作为二级存储上的二进制文件来进行管理的。经常会保存经过压缩的索引来达到减少从二进制存储加载的
数据量，提升检索处理效率的目的。


* 索引检索器

索引检索器是利用索引进行全文搜索处理的组件。索引检索器根据来自搜索应用程序用户的查询，协同索引管理器进行检索处理。
在大多数情况下，索引检索器都会根据某种标准对查询相匹配的检索结果排序，并将排序在前面的结果返回给引用程序。


* 索引构建器

索引构建器，是从作为索引对象的文本文档中生成索引的组件。索引构建器会先通过解析将文本文档分解为单词序列，然后再将该单词序列转换为索引结构。

在搜索引擎中，将生成索引的环节称为索引构建(Index Construction).

* 文档管理器

文档管理器是管理文档数据库的组件，文档数据库中存储着作为检索对象的文档。文档管理器会先从文档数据库中取出与查询相匹配的文档，然后根据需要从该文档中提取出一部分内容作为摘要。

文档管理器的结构非常简单，只是对应着文档特定的ID（文档编号）来保存文档的内容。

由于文档管理器管理的文档数据库即可作为在构建索引的阶段随索引一同构建，也可以提前构建。

### 与搜索引擎相关的组件5

* 爬虫

爬虫是用于收集WEB上的HTML文件等文档的系统。


* 搜索排序系统

以Google的PageRank系统为代表的搜索排序系统是给作为检索对象的文档打分的系统。例如，在WEB检索中，通常会以考量了查询与文档的关联性以及文档的热门度后得出的分数为基准，将检索结果排序后提供给应用程序的用户。搜索排序系统正是用于此目的，能机械算出文档热门度的系统。


## 实现快速全文搜索的索引结构

## 深入理解倒排索引

## 制作中文文档的倒排索引

## 实现倒排索引

## 使用倒排所索引进行检索

## 构建倒排索引

## 准备要检索的文档





