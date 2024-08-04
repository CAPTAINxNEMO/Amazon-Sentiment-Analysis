from vaderSentiment.vaderSentiment import SentimentIntensityAnalyzer
from csv import reader

sentimentIntensityAnalyzer = SentimentIntensityAnalyzer()

def sentimentScores(review):
    global countPositive
    global countNegative
    global countNeutral
    sentimentDict = sentimentIntensityAnalyzer.polarity_scores(review)
    print(f"\nOverall sentiment dictionary is: ", sentimentDict)
    print(f"Review was rated as {sentimentDict['neg'] * 100:.4f} % Negative")
    print(f"Review was rated as {sentimentDict['neu'] * 100:.4f} % Neutral")
    print(f"Review was rated as {sentimentDict['pos'] * 100:.4f} % Positive")
    print("\nReview Overall Rated As", end = " ")
    if sentimentDict['compound'] >= 0.05:
        countPositive += 1
        print("Positive")
    elif sentimentDict['compound'] <= -0.05:
        countNegative += 1
        print("Negative")
    else:
        countNeutral += 1
        print("Neutral")

def averageSentiment(wm_content):
    positive_scores = []
    negative_scores = []
    neutral_scores = []
    for review in wm_content:
        sentimentScores = sentimentIntensityAnalyzer.polarity_scores(review)
        positive_scores.append(sentimentScores['pos'])
        negative_scores.append(sentimentScores['neg'])
        neutral_scores.append(sentimentScores['neu'])
    # print(f"\nOverall  : ", sentimentScores)
    # print(f"Overall Negative : {(sum(negative_scores) / len(negative_scores)) * 100:.4f} %")
    # print(f"Overall Neutral : {(sum(neutral_scores) / len(neutral_scores)) * 100:.4f} %")
    # print(f"Overall Positive : {(sum(positive_scores) / len(positive_scores)) * 100:.4f} %")
    print("\nAverage Rated As", end = " ")
    if sentimentScores['compound'] >= 0.05:
        print("Positive\nMost buyers like this product.")
    elif sentimentScores['compound'] <= - 0.05:
        print("Negative\nMost buyers don't like this product.")
    else:
        print("Neutral\nSome buyers like this product while others don't.")

if __name__ == "__main__":
    filePath = 'Reviews.csv'
    with open(filePath, mode='r', newline='') as file:
        CSVReader = reader(file)
        rowCount = sum(1 for row in CSVReader)
    countPositive = 0
    countNegative = 0
    countNeutral = 0
    # review_content = []
    # for review_text in review_content:
    #     print(review_text)
    #     sentimentScores(review_text)
    #     print("=" * 50)
    #     '''
    #     if i == n:
    #         print("#" * 50)
    #         print(f"Number of reviews: {len(wm_content)}")
    #         print(f"Number of Positive Reviews: {countPositive}")
    #         print(f"Number of Negative Reviews: {countNegative}")
    #         print(f"Number of Neutral Reviews: {countNeutral}")
    #         averageSentiment(wm_content)
    #         print("#" * 50)
    #     '''
    # print("#" * 50)
    print(f"Number of reviews: {rowCount}")
    print(f"Number of Positive Reviews: {countPositive}")
    print(f"Number of Negative Reviews: {countNegative}")
    print(f"Number of Neutral Reviews: {countNeutral}")
    # averageSentiment(wm_content)
    # print("#" * 50)